#!/usr/bin/env python3
"""Stage the source-pinned native runtime; never fetch game jars or prepared resources."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import shutil
import zipfile

MANIFEST = Path(__file__).resolve().parents[2] / 'launcher/runtime-manifest.json'


def digest(path):
    result = hashlib.sha256()
    with path.open('rb') as f:
        for block in iter(lambda: f.read(1024 * 1024), b''):
            result.update(block)
    return result.hexdigest()


def stage(platform, source, output, manifest=MANIFEST):
    info = json.loads(manifest.read_text())['platforms'][platform]
    source = Path(source) / info['asset']
    if digest(source) != info['sha256'] or source.stat().st_size != info['size']:
        raise ValueError('Native runtime does not match the reviewed source-pinned package.')
    output = Path(output)
    output.parent.mkdir(parents=True, exist_ok=True)
    shutil.copyfile(source, output)
    return output


def signing_inputs(pack, output):
    output = Path(output)
    output.mkdir(parents=True, exist_ok=True)
    with zipfile.ZipFile(pack) as archive:
        for name in ['horizon-client.exe', 'horizon-prepare.exe']:
            found = [n for n in archive.namelist() if n.endswith('/' + name)]
            if len(found) != 1:
                raise ValueError('Expected one native executable: ' + name)
            (output / name).write_bytes(archive.read(found[0]))


def apply_signatures(pack, signed):
    """Only called after Windows verified the returned provider signatures."""
    pack, signed = Path(pack), Path(signed)
    replacement = {n: (signed / n).read_bytes() for n in ['horizon-client.exe', 'horizon-prepare.exe']}
    temporary = pack.with_suffix('.signed')
    matched = []
    with zipfile.ZipFile(pack) as source, zipfile.ZipFile(temporary, 'w', zipfile.ZIP_DEFLATED) as dest:
        for entry in source.infolist():
            name = entry.filename.rsplit('/', 1)[-1]
            data = source.read(entry)
            if name in replacement:
                data = replacement[name]
                matched.append(name)
            dest.writestr(entry, data)
    if sorted(matched) != sorted(replacement):
        temporary.unlink()
        raise ValueError('The package must contain exactly the two signed native executables.')
    temporary.replace(pack)


def build_environment(pack, destination):
    # These immutable build-time values bind the resulting launcher to this exact package.
    with Path(destination).open('a') as output:
        output.write('NIROVA_BUNDLED_RUNTIME_SHA256=' + digest(pack) + '\n')
        output.write('NIROVA_BUNDLED_RUNTIME_SIZE=' + str(pack.stat().st_size) + '\n')


def main():
    p = argparse.ArgumentParser()
    p.add_argument('--platform', choices=['windows', 'linux', 'macos', 'macos-x64'])
    p.add_argument('--input', type=Path)
    p.add_argument('--output', required=True, type=Path)
    p.add_argument('--signing-input', type=Path)
    p.add_argument('--signed-files', type=Path)
    p.add_argument('--github-env', type=Path)
    args = p.parse_args()
    if args.input:
        stage(args.platform, args.input, args.output)
    if args.signing_input:
        signing_inputs(args.output, args.signing_input)
    if args.signed_files:
        apply_signatures(args.output, args.signed_files)
    if args.github_env:
        build_environment(args.output, args.github_env)


if __name__ == '__main__':
    main()

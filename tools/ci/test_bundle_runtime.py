import hashlib
import json
from pathlib import Path
import tempfile
import unittest
import zipfile
from bundle_runtime import stage, signing_inputs, apply_signatures, build_environment, digest


class RuntimePackageTests(unittest.TestCase):
    def test_package_integrity_is_required_before_staging(self):
        with tempfile.TemporaryDirectory() as root:
            root = Path(root)
            archive = root / 'game.zip'
            archive.write_bytes(b'reviewed native game archive')
            manifest = root / 'manifest.json'
            manifest.write_text(json.dumps({'platforms': {'windows': {'asset': 'game.zip', 'sha256': digest(archive), 'size': archive.stat().st_size}}}))
            target = root / 'output' / 'runtime.pack'
            stage('windows', root, target, manifest)
            self.assertEqual(target.read_bytes(), archive.read_bytes())
            archive.write_bytes(b'altered executable')
            with self.assertRaises(ValueError):
                stage('windows', root, target, manifest)
            self.assertEqual(target.read_bytes(), b'reviewed native game archive')

    def test_signed_runtime_repack_preserves_resources_and_binds_new_launcher_digest(self):
        with tempfile.TemporaryDirectory() as root:
            root = Path(root)
            pack = root / 'runtime.pack'
            with zipfile.ZipFile(pack, 'w') as z:
                z.writestr('game/horizon-client.exe', b'unsigned-client')
                z.writestr('game/horizon-prepare.exe', b'unsigned-preparer')
                z.writestr('game/research-lock.json', b'resources-metadata')
            signing_inputs(pack, root / 'inputs')
            self.assertEqual((root / 'inputs/horizon-client.exe').read_bytes(), b'unsigned-client')
            signed = root / 'signed'
            signed.mkdir()
            (signed / 'horizon-client.exe').write_bytes(b'provider-signed-client')
            (signed / 'horizon-prepare.exe').write_bytes(b'provider-signed-preparer')
            old_digest = digest(pack)
            apply_signatures(pack, signed)
            with zipfile.ZipFile(pack) as z:
                self.assertEqual(z.read('game/research-lock.json'), b'resources-metadata')
                self.assertEqual(z.read('game/horizon-client.exe'), b'provider-signed-client')
            env = root / 'env'
            build_environment(pack, env)
            self.assertNotEqual(old_digest, digest(pack))
            self.assertIn('NIROVA_BUNDLED_RUNTIME_SHA256=' + digest(pack), env.read_text())


if __name__ == '__main__':
    unittest.main()

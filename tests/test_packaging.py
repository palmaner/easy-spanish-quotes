"""Output isolation checks; these tests never build or install a keyboard."""
import importlib.util
from pathlib import Path
import tempfile
import unittest
from unittest.mock import patch

spec=importlib.util.spec_from_file_location('package_prototype',Path(__file__).resolve().parents[1]/'tools/package-prototype.py')
packaging=importlib.util.module_from_spec(spec)
spec.loader.exec_module(packaging)


class PackagingTests(unittest.TestCase):
    def temporary_workspace(self):
        parent=Path(__file__).resolve().parents[1]/'build'/'packaging-tests'
        parent.mkdir(parents=True,exist_ok=True)
        temporary=tempfile.TemporaryDirectory(dir=parent)
        # Verify the cleanup target stays in the dedicated workspace directory.
        self.assertTrue(Path(temporary.name).resolve().is_relative_to(parent.resolve()))
        return temporary

    def test_revision_cannot_escape_dist_or_create_nested_output(self):
        for revision in ('../old','a/b','a\\b','C:other','-bad','', 'x'*41,'á','new.zip'):
            with self.subTest(revision=revision), self.assertRaises(ValueError):
                packaging.package_paths(Path('workspace'),revision)

    def test_existing_archive_is_preserved_before_any_build_tool_is_called(self):
        with self.temporary_workspace() as directory:
            root=Path(directory)
            destination, archive=packaging.package_paths(root,'test')
            archive.parent.mkdir()
            archive.write_bytes(b'previous artifact')
            with patch.object(packaging,'ROOT',root), patch.object(packaging.subprocess,'check_output') as command:
                with self.assertRaises(FileExistsError): packaging.package('cargo','test')
                command.assert_not_called()
            self.assertEqual(archive.read_bytes(),b'previous artifact')
            self.assertFalse(destination.exists())

    def test_existing_directory_is_preserved(self):
        with self.temporary_workspace() as directory:
            root=Path(directory)
            destination, _=packaging.package_paths(root)
            destination.mkdir(parents=True)
            marker=destination/'user-file.txt'
            marker.write_text('keep',encoding='utf-8')
            with patch.object(packaging,'ROOT',root), self.assertRaises(FileExistsError):
                packaging.package('cargo')
            self.assertEqual(marker.read_text(encoding='utf-8'),'keep')

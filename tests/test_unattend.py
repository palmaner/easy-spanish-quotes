import importlib.util
from io import StringIO
from pathlib import Path
import unittest

spec = importlib.util.spec_from_file_location(
    "check_unattend", Path(__file__).resolve().parents[1] / "tools/check-unattend.py")
module = importlib.util.module_from_spec(spec)
spec.loader.exec_module(module)


class AnswerFilePreflightTests(unittest.TestCase):
    def check(self, group="Administrators", extra="", account="esq-test"):
        xml = f'''<unattend xmlns="urn:schemas-microsoft-com:unattend">
          <settings><component>{extra}<UserAccounts><LocalAccounts>
          <LocalAccount><Name>{account}</Name><Group>{group}</Group>
          <Password><Value>TEST_ONLY_NOT_A_CREDENTIAL</Value></Password>
          </LocalAccount></LocalAccounts></UserAccounts></component></settings>
          </unattend>'''
        return module.check_answer_file(StringIO(xml))

    def test_canonical_admin_group_passes(self):
        self.assertEqual(self.check(), [])
        self.assertEqual(self.check(group="Administrators;Users"), [])

    def test_lowercase_or_standard_group_does_not_establish_admin_access(self):
        for group in ("administrators;users", "Users", " Administrators"):
            with self.subTest(group=group):
                self.assertTrue(self.check(group=group))

    def test_deprecated_network_location_rejected(self):
        self.assertTrue(self.check(extra="<NetworkLocation>Public</NetworkLocation>"))

    def test_unexpected_account_rejected(self):
        self.assertTrue(self.check(account="other-account"))

    def test_parse_failure_does_not_disclose_input(self):
        errors = module.check_answer_file(StringIO("PRIVATE-CREDENTIAL<"))
        self.assertTrue(errors)
        self.assertNotIn("PRIVATE-CREDENTIAL", str(errors))


if __name__ == "__main__":
    unittest.main()

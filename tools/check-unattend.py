#!/usr/bin/env python3
"""Check a generated Windows answer file before provisioning the test VM.

Never print answer-file contents: they can contain local-account credentials.
This checks configuration, not successful installation or guest privileges.
"""
import argparse
import sys
import xml.etree.ElementTree as ET


def check_answer_file(path, test_account="esq-test"):
    try:
        root = ET.parse(path).getroot()
    except (OSError, ET.ParseError):
        return ["Answer file could not be read as XML."]
    errors = []
    if any(node.tag.rsplit("}", 1)[-1] == "NetworkLocation" for node in root.iter()):
        errors.append("Remove the deprecated NetworkLocation setting.")
    accounts = [node for node in root.iter()
                if node.tag.rsplit("}", 1)[-1] == "LocalAccount"]
    matches = []
    for account in accounts:
        children = {node.tag.rsplit("}", 1)[-1]: node for node in account}
        name = children.get("Name")
        if name is not None and name.text == test_account:
            matches.append(children)
    if len(matches) != 1:
        errors.append("Expected exactly one configured test account.")
    else:
        group = matches[0].get("Group")
        groups = (group.text or "").split(";") if group is not None else []
        # Use Microsoft's documented language-neutral spelling. Do not assume
        # lower-case aliases will be localized by Windows Setup.
        if "Administrators" not in groups:
            errors.append("The test account must specify the canonical Administrators group.")
        if any(value.strip() != value for value in groups):
            errors.append("Group names must not contain surrounding whitespace.")
        password = matches[0].get("Password")
        values = [] if password is None else [node for node in password
                  if node.tag.rsplit("}", 1)[-1] == "Value"]
        if len(values) != 1 or not values[0].text:
            errors.append("The test account needs a configured password.")
    return errors


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("answer_file")
    parser.add_argument("--test-account", default="esq-test")
    args = parser.parse_args()
    errors = check_answer_file(args.answer_file, args.test_account)
    if errors:
        for error in errors:
            print(error, file=sys.stderr)
        return 1
    print("Test VM answer-file preflight passed; guest validation remains required.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

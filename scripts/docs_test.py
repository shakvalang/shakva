"""Tests of scripts/docs.py: python3 -m unittest discover -s scripts -p '*_test.py'"""

import tempfile
import unittest
from pathlib import Path

import docs
from docs import Adr, DocsError


def adr_dir(files):
    directory = Path(tempfile.mkdtemp())
    for name, text in files.items():
        (directory / name).write_text(text, encoding="utf-8")
    return directory


ONE = "# ADR-0001: Files are ropes\n\nStatus: accepted, 2026-09-01\n\n## Context\n"
TWO = ("# ADR-0002: Ropes are stamped\n\nStatus: accepted, 2026-09-02\n"
       "Amends ADR-0001: stamps\n\n## Context\n\nAmends ADR-0001: not in the header\n")


class ReadAdrsTest(unittest.TestCase):
    def test_a_header_gives_the_number_the_decision_the_status_and_what_it_amends(self):
        # arrange
        directory = adr_dir({"0001-ropes.md": ONE, "0002-stamps.md": TWO, "README.md": "x"})
        # act
        adrs = docs.read_adrs(directory)
        # assert
        self.assertEqual(adrs, [
            Adr(1, "0001-ropes.md", "Files are ropes", "accepted", []),
            Adr(2, "0002-stamps.md", "Ropes are stamped", "accepted", [(1, "stamps")]),
        ])

    def test_a_superseded_adr_is_shown_by_the_d_of_the_one_replacing_it(self):
        # arrange
        old = ONE.replace("accepted, 2026-09-01", "superseded by ADR-0002, 2026-09-02")
        directory = adr_dir({"0001-ropes.md": old, "0002-stamps.md": TWO.replace("Amends", "x")})
        # act
        adrs = docs.read_adrs(directory)
        # assert
        self.assertEqual(adrs[0].status, "superseded by D2")

    def test_two_files_of_one_number_are_an_error_naming_both(self):
        # arrange
        directory = adr_dir({"0001-ropes.md": ONE, "0001-trees.md": ONE})
        # act, assert
        with self.assertRaisesRegex(DocsError, "ADR-0001 twice: 0001-ropes.md and 0001-trees.md"):
            docs.read_adrs(directory)

    def test_a_header_whose_number_is_not_its_files_is_an_error(self):
        directory = adr_dir({"0003-ropes.md": ONE})
        with self.assertRaisesRegex(DocsError, "its header says ADR-0001"):
            docs.read_adrs(directory)

    def test_a_header_without_a_status_is_an_error(self):
        directory = adr_dir({"0001-ropes.md": "# ADR-0001: Files are ropes\n\n## Context\n"})
        with self.assertRaisesRegex(DocsError, "no `Status"):
            docs.read_adrs(directory)

    def test_amending_an_adr_that_is_not_there_is_an_error(self):
        directory = adr_dir({"0002-stamps.md": TWO})
        with self.assertRaisesRegex(DocsError, "names ADR-0001, which is not another ADR"):
            docs.read_adrs(directory)


class IndexTest(unittest.TestCase):
    def test_an_amended_adr_shows_the_amending_ones_in_its_row_by_number(self):
        # arrange
        adrs = [
            Adr(1, "0001-a.md", "A", "accepted", []),
            Adr(2, "0002-b.md", "B", "accepted", [(1, "stamps")]),
            Adr(3, "0003-c.md", "C", "accepted", [(1, "a re-export is its package's, not its file's")]),
        ]
        # act
        rows = docs.index_rows(adrs)
        # assert
        self.assertEqual(rows[0], "| [0001](0001-a.md) | A | accepted; stamps by D2; "
                                  "a re-export is its package's, not its file's, by D3 |")
        self.assertEqual(rows[1], "| [0002](0002-b.md) | B | accepted |")

    def test_the_table_is_replaced_and_what_is_around_it_kept(self):
        # arrange
        text = f"# ADR\n\nintro\n\n{docs.TABLE_HEAD}\n{docs.TABLE_RULE}\n| old | row | x |\n\nafter\n"
        # act
        once = docs.write_index(text, ["| new | row | y |"])
        twice = docs.write_index(once, ["| new | row | y |"])
        # assert
        self.assertEqual(once, f"# ADR\n\nintro\n\n{docs.GENERATED}\n\n{docs.TABLE_HEAD}\n"
                               f"{docs.TABLE_RULE}\n| new | row | y |\n\nafter\n")
        self.assertEqual(twice, once)


class RulesTest(unittest.TestCase):
    def test_a_rule_id_twice_in_its_file_is_an_error_struck_through_or_not(self):
        # arrange
        root = Path(tempfile.mkdtemp())
        for prefix, name in docs.RULES.items():
            (root / name).parent.mkdir(parents=True, exist_ok=True)
            (root / name).write_text(f"| {prefix}1 | a |\n| ~~{prefix}2~~ | b |\n", encoding="utf-8")
        (root / docs.RULES["C"]).write_text("| C1 | a |\n| ~~C1~~ | b |\n| L1 | not C's |\n")
        # act, assert
        with self.assertRaisesRegex(DocsError, "01-conventions.md: C1 twice"):
            docs.check_rules(root)

    def test_the_rules_of_this_repository_have_each_id_once(self):
        docs.check_rules()


class OpenPrsTest(unittest.TestCase):
    ADRS = [Adr(56, "0056-mine.md", "Mine", "proposed", [])]

    def test_an_older_pull_request_with_the_number_fails_the_younger(self):
        with self.assertRaisesRegex(DocsError, r"ADR-0056 is #170's \(0056-theirs.md\)"):
            docs.check_open_prs(171, self.ADRS, {170: [(56, "0056-theirs.md")]})

    def test_a_younger_pull_request_with_the_number_is_its_own_concern(self):
        docs.check_open_prs(170, self.ADRS, {171: [(56, "0056-theirs.md")]})

    def test_an_older_pull_request_editing_the_same_adr_is_no_clash(self):
        docs.check_open_prs(171, self.ADRS, {170: [(56, "0056-mine.md")]})


if __name__ == "__main__":
    unittest.main()

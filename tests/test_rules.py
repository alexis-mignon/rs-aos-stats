import pytest
from rs_aos_stats import (
    HitRule, WoundRule, SaveRule, DamagesRule, AttackCharacteristicRule,
    WardRule, CritAutoWoundRule, CritMortalWoundRule, CritDoubleHitRule,
)

ALL_RULE_CLASSES = [
    HitRule,
    WoundRule,
    SaveRule,
    DamagesRule,
    AttackCharacteristicRule,
    WardRule,
    CritAutoWoundRule,
    CritMortalWoundRule,
    CritDoubleHitRule,
]


class TestRules:
    """Tests for rule instantiation."""

    @pytest.mark.parametrize("rule_cls", ALL_RULE_CLASSES)
    def test_instantiate_rule(self, rule_cls):
        """Each rule class should instantiate without error."""
        rule = rule_cls()
        assert rule is not None

    def test_all_nine_rule_types(self):
        """There should be exactly 9 rule types."""
        assert len(ALL_RULE_CLASSES) == 9

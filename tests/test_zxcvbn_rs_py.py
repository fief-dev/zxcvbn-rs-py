import pytest

from zxcvbn_rs_py import Score, zxcvbn


@pytest.mark.parametrize(
    "password,score",
    [
        ("correcthorsebatterystaple", Score.FOUR),
    ],
)
def test_zxcvbn_rs_py(password: str, score: Score) -> None:
    r = zxcvbn("correcthorsebatterystaple")
    assert r.score == score

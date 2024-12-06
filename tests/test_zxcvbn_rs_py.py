import pytest

from zxcvbn_rs_py import zxcvbn


@pytest.mark.parametrize(
    "password,score",
    [
        ("correcthorsebatterystaple", 4),
    ],
)
def test_zxcvbn_rs_py(password: str, score: int) -> None:
    r = zxcvbn("correcthorsebatterystaple")
    assert r.score == score

import json

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


def test_entropy_to_dict_shape() -> None:
    result = zxcvbn("correcthorsebatterystaple").to_dict()
    assert set(result.keys()) == {
        "guesses",
        "guesses_log10",
        "crack_times_seconds",
        "crack_times_display",
        "score",
        "feedback",
        "calc_time",
    }
    assert isinstance(result["score"], int)
    assert isinstance(result["crack_times_seconds"], dict)
    assert isinstance(result["crack_times_display"], dict)


def test_entropy_to_dict_json_serializable() -> None:
    weak_result = zxcvbn("password").to_dict()
    strong_result = zxcvbn("correcthorsebatterystaple").to_dict()

    json.dumps(weak_result)
    json.dumps(strong_result)

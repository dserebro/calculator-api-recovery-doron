"""Integration tests for Milestone 1: Health check and arithmetic endpoints."""
import requests

BASE_URL = "http://localhost:8000"


class TestHealthCheck:
    """Tests for GET /health endpoint."""

    def test_health_returns_200(self):
        resp = requests.get(f"{BASE_URL}/health")
        assert resp.status_code == 200

    def test_health_json_shape(self):
        resp = requests.get(f"{BASE_URL}/health")
        data = resp.json()
        assert data["status"] == "ok"
        assert data["version"] == "0.1.0"


class TestAdd:
    """Tests for POST /add endpoint."""

    def test_add_positive(self):
        resp = requests.post(f"{BASE_URL}/add", json={"a": 2.0, "b": 3.0})
        assert resp.status_code == 200
        assert resp.json()["result"] == 5.0

    def test_add_negative(self):
        resp = requests.post(f"{BASE_URL}/add", json={"a": -1.0, "b": -2.0})
        assert resp.status_code == 200
        assert resp.json()["result"] == -3.0

    def test_add_zero(self):
        resp = requests.post(f"{BASE_URL}/add", json={"a": 0.0, "b": 0.0})
        assert resp.status_code == 200
        assert resp.json()["result"] == 0.0

    def test_add_integers(self):
        """Python Pydantic accepts ints for float fields; serde should too."""
        resp = requests.post(f"{BASE_URL}/add", json={"a": 2, "b": 3})
        assert resp.status_code == 200
        assert resp.json()["result"] == 5.0


class TestSubtract:
    """Tests for POST /subtract endpoint."""

    def test_subtract_positive(self):
        resp = requests.post(f"{BASE_URL}/subtract", json={"a": 5.0, "b": 3.0})
        assert resp.status_code == 200
        assert resp.json()["result"] == 2.0

    def test_subtract_negative_result(self):
        resp = requests.post(f"{BASE_URL}/subtract", json={"a": 3.0, "b": 5.0})
        assert resp.status_code == 200
        assert resp.json()["result"] == -2.0


class TestMultiply:
    """Tests for POST /multiply endpoint."""

    def test_multiply_positive(self):
        resp = requests.post(f"{BASE_URL}/multiply", json={"a": 2.0, "b": 3.0})
        assert resp.status_code == 200
        assert resp.json()["result"] == 6.0

    def test_multiply_by_zero(self):
        resp = requests.post(f"{BASE_URL}/multiply", json={"a": 5.0, "b": 0.0})
        assert resp.status_code == 200
        assert resp.json()["result"] == 0.0

    def test_multiply_negative(self):
        resp = requests.post(f"{BASE_URL}/multiply", json={"a": -2.0, "b": 3.0})
        assert resp.status_code == 200
        assert resp.json()["result"] == -6.0


class TestDivide:
    """Tests for POST /divide endpoint."""

    def test_divide_positive(self):
        resp = requests.post(f"{BASE_URL}/divide", json={"a": 6.0, "b": 3.0})
        assert resp.status_code == 200
        assert resp.json()["result"] == 2.0

    def test_divide_fractional(self):
        resp = requests.post(f"{BASE_URL}/divide", json={"a": 1.0, "b": 3.0})
        assert resp.status_code == 200
        result = resp.json()["result"]
        assert abs(result - 1.0 / 3.0) < 1e-10

    def test_divide_by_zero_returns_400(self):
        resp = requests.post(f"{BASE_URL}/divide", json={"a": 5.0, "b": 0.0})
        assert resp.status_code == 400
        assert resp.json()["detail"] == "Cannot divide by zero"

    def test_divide_zero_by_nonzero(self):
        resp = requests.post(f"{BASE_URL}/divide", json={"a": 0.0, "b": 5.0})
        assert resp.status_code == 200
        assert resp.json()["result"] == 0.0


class TestErrorHandling:
    """Tests for error cases."""

    def test_missing_field_returns_error(self):
        resp = requests.post(f"{BASE_URL}/add", json={"a": 5.0})
        # Axum returns 422 for deserialization errors (vs FastAPI's 422)
        assert resp.status_code in (400, 422)

    def test_invalid_json_returns_error(self):
        resp = requests.post(
            f"{BASE_URL}/add",
            data="not json",
            headers={"Content-Type": "application/json"},
        )
        assert resp.status_code in (400, 422)

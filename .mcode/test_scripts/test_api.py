"""Functional tests for the Calculator API (Milestone 1 - Stateless Endpoints)."""
import requests

BASE_URL = "http://localhost:8000"


# --- Health Check ---

def test_health_returns_200():
    resp = requests.get(f"{BASE_URL}/health")
    assert resp.status_code == 200


def test_health_response_shape():
    resp = requests.get(f"{BASE_URL}/health")
    data = resp.json()
    assert data == {"status": "ok", "version": "0.1.0"}


# --- POST /add ---

def test_add_positive_numbers():
    resp = requests.post(f"{BASE_URL}/add", json={"a": 5, "b": 3})
    assert resp.status_code == 200
    assert resp.json() == {"result": 8.0}


def test_add_negative_numbers():
    resp = requests.post(f"{BASE_URL}/add", json={"a": -2, "b": -3})
    assert resp.status_code == 200
    assert resp.json() == {"result": -5.0}


def test_add_with_zero():
    resp = requests.post(f"{BASE_URL}/add", json={"a": 0, "b": 5})
    assert resp.status_code == 200
    assert resp.json() == {"result": 5.0}


def test_add_mixed_signs():
    resp = requests.post(f"{BASE_URL}/add", json={"a": -2, "b": 3})
    assert resp.status_code == 200
    assert resp.json() == {"result": 1.0}


# --- POST /subtract ---

def test_subtract_positive_numbers():
    resp = requests.post(f"{BASE_URL}/subtract", json={"a": 10, "b": 3})
    assert resp.status_code == 200
    assert resp.json() == {"result": 7.0}


def test_subtract_negative_result():
    resp = requests.post(f"{BASE_URL}/subtract", json={"a": 3, "b": 10})
    assert resp.status_code == 200
    assert resp.json() == {"result": -7.0}


def test_subtract_with_zero():
    resp = requests.post(f"{BASE_URL}/subtract", json={"a": 5, "b": 0})
    assert resp.status_code == 200
    assert resp.json() == {"result": 5.0}


# --- POST /multiply ---

def test_multiply_positive_numbers():
    resp = requests.post(f"{BASE_URL}/multiply", json={"a": 7, "b": 6})
    assert resp.status_code == 200
    assert resp.json() == {"result": 42.0}


def test_multiply_with_zero():
    resp = requests.post(f"{BASE_URL}/multiply", json={"a": 5, "b": 0})
    assert resp.status_code == 200
    assert resp.json() == {"result": 0.0}


def test_multiply_negative_numbers():
    resp = requests.post(f"{BASE_URL}/multiply", json={"a": -3, "b": -4})
    assert resp.status_code == 200
    assert resp.json() == {"result": 12.0}


# --- POST /divide ---

def test_divide_happy_path():
    resp = requests.post(f"{BASE_URL}/divide", json={"a": 10, "b": 2})
    assert resp.status_code == 200
    assert resp.json() == {"result": 5.0}


def test_divide_fractional_result():
    resp = requests.post(f"{BASE_URL}/divide", json={"a": 10, "b": 3})
    assert resp.status_code == 200
    assert resp.json() == {"result": 3.3333333333333335}


def test_divide_negative_numbers():
    resp = requests.post(f"{BASE_URL}/divide", json={"a": -6, "b": -3})
    assert resp.status_code == 200
    assert resp.json() == {"result": 2.0}


def test_divide_by_zero_returns_400():
    resp = requests.post(f"{BASE_URL}/divide", json={"a": 5, "b": 0})
    assert resp.status_code == 400


def test_divide_by_zero_error_body():
    resp = requests.post(f"{BASE_URL}/divide", json={"a": 5, "b": 0})
    data = resp.json()
    assert data == {"detail": "Cannot divide by zero"}


def test_divide_zero_by_zero_returns_400():
    resp = requests.post(f"{BASE_URL}/divide", json={"a": 0, "b": 0})
    assert resp.status_code == 400
    assert resp.json() == {"detail": "Cannot divide by zero"}


# --- Float serialization ---

def test_integer_results_serialized_as_floats():
    """Verify that integer-like results include the .0 suffix (e.g., 8.0 not 8)."""
    resp = requests.post(f"{BASE_URL}/add", json={"a": 5, "b": 3})
    # The raw text should contain "8.0" not just "8"
    assert "8.0" in resp.text

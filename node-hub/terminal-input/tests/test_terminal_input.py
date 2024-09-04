import pytest


def test_import_main():
    from terminal_input.main import main

    # Check that everything is working, and catch dora Runtime Exception as we're not running in a dora dataflow.
    with pytest.raises(RuntimeError):
        main()


if __name__ == "__main__":
    # Call pytest.main() to run tests
    pytest.main(["-s", "-v"])

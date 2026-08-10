VENV := .venv
PYTHON := $(VENV)/bin/python
REQUIREMENTS := scripts/requirements.txt
REQUIREMENTS_STAMP := $(VENV)/.requirements-installed

$(PYTHON):
	python3 -m venv $(VENV)

$(REQUIREMENTS_STAMP): $(REQUIREMENTS) $(PYTHON)
	$(PYTHON) -m pip install -r $(REQUIREMENTS)
	@touch $(REQUIREMENTS_STAMP)

.PHONY: favicons
favicons: $(REQUIREMENTS_STAMP)
	cd ../.. && sites/polgarand.org/.venv/bin/python sites/polgarand.org/scripts/download_favicons.py

#!/Library/Frameworks/Python.framework/Versions/3.11/bin/python3.11
# -*- coding: utf-8 -*-
import re
import sys
from skystore_cli import app
if __name__ == '__main__':
    sys.argv[0] = re.sub(r'(-script\.pyw|\.exe)?$', '', sys.argv[0])
    sys.exit(app())

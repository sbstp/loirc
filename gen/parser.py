def capfirst(s):
    return s[:1].upper() + s[1:].lower()

class Code:

    def __init__(self, code, value):
        self.code = code
        self.value = value
        self.reply = code.startswith("RPL_")
        self.error = code.startswith("ERR_")
        self.format_code = self._format_code()
        self.format_value = self._format_value()

    def _format_code(self):
        if self.reply or self.error:
            parts = self.code.split("_")
            return capfirst(parts[0]) + capfirst(parts[1])
        else:
            return capfirst(self.code)

    def _format_value(self):
        return '"' + self.value.upper() + '"'


def parse(path):
    f = open(path, "r")
    codes = []
    while True:
        code = f.readline()
        if not code: break
        value = f.readline()
        if not value: break
        codes.append(Code(code.strip(), value.strip()))
    return codes

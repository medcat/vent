import sys

if (sys.version_info > (3, 0)):
    def as_bytes(str):
        return bytes(str, "ASCII")
else:
    def as_bytes(str):
        return str

def main():
	finalsize = 64 * 2 * 1024
	f = open("payload.bin", "rb")
	raw = f.read()
	f.close()
	f = open("bios.bin", "wb")
	f.write(as_bytes("\0") * (finalsize - len(raw)) + raw)
	f.close()

if __name__ == '__main__':
	main()

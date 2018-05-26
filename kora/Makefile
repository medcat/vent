bios.bin: payload.bin pad.py
	python pad.py

payload.bin: payload.o payload.ld
	ld -o payload.bin -T payload.ld

payload.o: payload.S
	as $< -o $@

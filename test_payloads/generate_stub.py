#!/usr/bin/env python3
import os

def main():
    stub = bytes([0x90] * 32 + [0xC3])
    
    script_dir = os.path.dirname(os.path.abspath(__file__))
    file_path = os.path.join(script_dir, "nop_stub.bin")
    
    with open(file_path, "wb") as f:
        f.write(stub)
        
    print("[+] Generated nop_stub.bin")
    print("[+] Contents: 32x NOP (0x90) + RET (0xC3) = 33 bytes")
    print("[+] Safe for local testing — returns immediately when called as a function")

if __name__ == "__main__":
    main()

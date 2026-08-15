#!/usr/bin/env python3

import socket
import sys

SOCKET = "/tmp/twitch-overlay.sock"


def send_command(command):
    with socket.socket(socket.AF_UNIX, socket.SOCK_STREAM) as sock:
        sock.connect(SOCKET)
        sock.sendall((command + "\n").encode())
        response = sock.recv(1024).decode().strip()

    print(response)


if __name__ == "__main__":
    if len(sys.argv) != 2:
        print("Usage: python3 control.py <command>")
        print()
        print("Commands:")
        print("  ping")
        print("  toggle")
        print("  enable")
        print("  disable")
        print("  show")
        print("  hide")
        print("  reload")
        print("  quit")
        sys.exit(1)

    send_command(sys.argv[1])

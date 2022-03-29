import json
import socket

from pydantic import parse_obj_as

from python.robot_comm_models import FieldPosition, FieldPose, Trajectory
from python.robot_sim_server import gen_trajectory

HOST = "127.0.0.1"  # Standard loopback interface address (localhost)
PORT = 65426  # Port to listen on (non-privileged ports are > 1023)

with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
    s.bind((HOST, PORT))
    s.listen()
    conn, addr = s.accept()
    with conn:
        print(f"Connected to client.")
        while True:
            data = conn.recv(2048)
            if not data:
                break

            d = json.loads(data)

            print(d)

            t: Trajectory = parse_obj_as(Trajectory, d)

            print(t)

            try:
                trajectory = gen_trajectory(t.start, t.points, t.end)
            except:
                trajectory = []

            conn.sendall(
                json.dumps([k.dict() for k in trajectory]).encode()
            )

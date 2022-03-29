from robotpy_toolkit_7407.utils.units import rad, m, s
from wpimath.geometry import Translation2d

from python.robot_comm_models import FieldPose, FieldPosition
from python.swerve_sim_trajectory import SimTrajectory, TrajectoryEndpoint


def gen_trajectory(start_pose: FieldPose, waypoints: list[FieldPosition], end_pose: FieldPose) -> list[FieldPosition]:
    trajectory = SimTrajectory.generate_trajectory(
        TrajectoryEndpoint(start_pose.translation.x * m, start_pose.translation.y * m, start_pose.rotation * rad),
        list(Translation2d(w.x, w.y) for w in waypoints),
        TrajectoryEndpoint(end_pose.translation.x * m, end_pose.translation.y * m, end_pose.rotation * rad),
        5 * m/s,
        1 * m/(s*s)
    )
    samples = 100
    step = trajectory.totalTime() / samples
    t = 0
    points = []
    while t <= trajectory.totalTime():
        sample = trajectory.sample(t)
        points.append(FieldPosition(x=sample.pose.X(), y=sample.pose.Y()))
        t += step
    return points

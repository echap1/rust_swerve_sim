from robotpy_toolkit_7407.utils.units import rad, m, s
from wpimath.geometry import Translation2d

from python.swerve_sim_trajectory import SimTrajectory, TrajectoryEndpoint


def gen_trajectory(
        start_pose: tuple[float, float, float],
        waypoints: list[tuple[float, float]],
        end_pose: tuple[float, float, float]) -> list[tuple[float, float]]:
    trajectory = SimTrajectory.generate_trajectory(
        TrajectoryEndpoint(start_pose[0] * m, start_pose[1] * m, start_pose[2] * rad),
        list(Translation2d(x, y) for x, y in waypoints),
        TrajectoryEndpoint(end_pose[0] * m, end_pose[1] * m, end_pose[2] * rad),
        0 * m/s,
        0 * m/s
    )
    samples = 1000
    step = trajectory.totalTime() / samples
    t = 0
    points = []
    while t <= trajectory.totalTime():
        sample = trajectory.sample(t)
        points.append((sample.pose.X(), sample.pose.Y()))
        t += step
    return points

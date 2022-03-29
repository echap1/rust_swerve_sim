from pydantic import BaseModel


class FieldPosition(BaseModel):
    x: float
    y: float


class FieldPose(BaseModel):
    translation: FieldPosition
    rotation: float


class Trajectory(BaseModel):
    start: FieldPose
    points: list[FieldPosition]
    end: FieldPose

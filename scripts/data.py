import enum
import json
import os

import numpy as np


class LaneAssociation(enum.Enum):
    UNKNOWN = 0
    RIGHT = 1
    CENTER = 2
    LEFT = 3


NUM_VEHICLES = 6
NUM_ITERATIONS = 3
FPS = 25

VEHICLE_FILENAME = "vehicle_data.json"
VEHICLE_FILEPATH = os.path.join(
    os.path.dirname(os.path.dirname(__file__)), "data", VEHICLE_FILENAME
)

EGO_FILENAME = "ego_data.json"
EGO_VEHICLE_FILEPATH = os.path.join(
    os.path.dirname(os.path.dirname(__file__)), "data", EGO_FILENAME
)


np.random.seed(0)


def kph_to_mps(kph: float) -> float:
    return kph / 3.6


def generate_vehicle_data(id_: int) -> dict:
    if id_ < 2:
        lane_data = int(LaneAssociation.LEFT.value)
        start_speed = np.random.uniform(110, 130)
    elif id_ < 4:
        lane_data = int(LaneAssociation.CENTER.value)
        start_speed = np.random.uniform(100, 110)
    else:
        lane_data = int(LaneAssociation.RIGHT.value)
        start_speed = np.random.uniform(80, 100)

    if id_ == 0 or id_ == 2 or id_ == 4:
        distance = np.random.uniform(20, 80)

    elif id_ == 1 or id_ == 3 or id_ == 5:
        distance = np.random.uniform(-80, -20)

    start_speed = kph_to_mps(start_speed)
    speed_data = [start_speed for _ in range(NUM_ITERATIONS)]
    for it in range(1, NUM_ITERATIONS):
        speed_data[it] = speed_data[it - 1] + np.random.normal(loc=0.0, scale=0.5)

    data_dict = {
        "lane": lane_data,
        "distance": distance,
        "speed": speed_data,
    }
    return data_dict


def main() -> int:
    vehicle_datas: dict[int, dict] = {idx: {} for idx in range(NUM_VEHICLES)}
    for i in range(NUM_VEHICLES):
        data = generate_vehicle_data(i)
        vehicle_datas[i] = data

    remapped_vehicle_data = {}
    for frame in range(NUM_ITERATIONS):
        frame_data = [
            {
                "lane": vehicle_datas[veh_id]["lane"],
                "id": veh_id,
                "distance": vehicle_datas[veh_id]["distance"],
                "speed": vehicle_datas[veh_id]["speed"][frame],
            }
            for veh_id in range(NUM_VEHICLES)
        ]

        remapped_vehicle_data[frame] = frame_data

    with open(VEHICLE_FILEPATH, "w") as file_object:
        json.dump(remapped_vehicle_data, file_object)

    ego_vehicle_data = {
        "lane": int(LaneAssociation.CENTER.value),
        "speed": kph_to_mps(np.random.uniform(110, 130)),
    }

    with open(EGO_VEHICLE_FILEPATH, "w") as file_object:
        json.dump(ego_vehicle_data, file_object)

    return 0


if __name__ == "__main__":
    main()

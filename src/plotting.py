import matplotlib.pyplot as plt
import argparse
import json
import numpy as np

if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("filename",  help="filename of the data to plot")
    args = parser.parse_args()

    with open(args.filename, "r") as f:
        data = json.load(f)
    
    xy_pairs = [(d['x0'], d['y0'], d['x1'], d['y1']) for d in data['pairs']]
    array = np.array(xy_pairs).reshape((-1, 2))
    print(f"shape: {array.shape}")
    print(array[:5, :])

    array = np.deg2rad(array)
    sin_phi = np.sin(array[:, 0])
    x = sin_phi * np.cos(array[:, 1])
    y = sin_phi * np.sin(array[:, 1])
    z = np.cos(array[:, 0])

    array = np.stack((x, y, z), axis=1)
    print(f"shape: {array.shape}")
    print(array[:5, :])

    fig = plt.figure()
    ax = fig.add_subplot(projection='3d')
    ax.scatter(x, y, z, c=z, cmap='viridis', linewidth=0.5)
    plt.show()
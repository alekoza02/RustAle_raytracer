import numpy as np
from utils import Image
from camera import Camera

W, H = 54, 54
post = Image(W,H)
camera = Camera(np.array([-3, -3, 4]), np.array([0.49960239, 0.49999984, -0.70738787]), np.array([-0.49999984, -0.50039761, -0.70682558]), np.array([0.70738787, -0.70682558, 0.]), np.pi/6)

prova = np.zeros((W,H,3))

for x in range(W):
    for y in range(H):

        camera.genera_direzione(x,y,W,H)
        
        prova[x,y,:] = [255 * camera.raggio_direzione[0], 255 * camera.raggio_direzione[1], 255]

post.salva(prova)
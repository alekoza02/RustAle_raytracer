import numpy as np

class Camera:
    def __init__(self, posizione, direzione, alto, destra, fov) -> None:
        self.origine = posizione
        self.dir = direzione
        self.ups = alto
        self.rig = destra
        self.fov = fov

    def genera_direzione(self, x, y, w, h):

        ndc_x = (2 * (x + np.random.random() * .5 - .5) - w) / w
        ndc_y = (h - 2 * (y + np.random.random() * .5 - .5)) / h

        screen_x = ndc_x * np.tan((self.fov) / 2)
        screen_y = ndc_y * np.tan((self.fov) / (2 * w/h))

        raggio_direzione = self.dir + screen_x * self.rig + screen_y * self.ups
        raggio_direzione /= np.linalg.norm(raggio_direzione)

        self.dir_iterazione = raggio_direzione

    def __str__(self) -> str:
        return "Camera"
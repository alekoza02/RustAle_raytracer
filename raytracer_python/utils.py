import numpy as np

class Image:
    def __init__(self, w, h) -> None:
        self.w, self.h = w, h
        self.max_color_value = 255
        self.header = f"P6\n{self.w} {self.h}\n{self.max_color_value}\n"

    def salva(self, data):
        with open("OUTPUT_python/python_1.ppm", "wb") as ppm_file:
            ppm_file.write(bytearray(self.header, "ascii"))
            data = data.transpose(1,0,2)
            data = data.ravel()
            ppm_file.write(data.astype(np.int8))
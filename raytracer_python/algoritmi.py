import numpy as np

class Collisioni:

    def __init__(self) -> None:
        pass

    @staticmethod
    def test_collisione(raggio, oggetti):
        
        risultato = HitInfo()
        migliore_dx = 100000

        for indice, oggetto in enumerate(oggetti):

            risultato_iter = oggetto.collisione_oggetto(raggio)

            if risultato_iter >= 0:

                if risultato_iter < migliore_dx:
                    
                    migliore_dx = risultato_iter

                    risultato.colpito = True
                    risultato.distanza = risultato_iter
                    risultato.indice_sfera = indice
                    risultato.punto_colpito = raggio.origine_iterante + raggio.dir_iterazione * risultato.distanza
                    risultato.norma_colpito = risultato.punto_colpito - oggetto.origine
                    risultato.norma_colpito /= np.linalg.norm(risultato.norma_colpito)

        return risultato

class HitInfo:

    def __init__(self) -> None:
        self.colpito = False
        self.punto_colpito = np.array([.0,.0,.0])
        self.norma_colpito = np.array([.0,.0,.0])
        self.norma_rifrazione = self.norma_colpito
        self.distanza = 0.0
        self.indice_sfera = 0
        self.front_face = False

    def check_front_face(self, raggio):
        if np.dot(raggio.dir_iterazione, self.norma_colpito) > 0:
            self.front_face = False
            self.norma_rifrazione = - self.norma_colpito
        else:
            self.front_face = True
            self.norma_rifrazione = self.norma_colpito
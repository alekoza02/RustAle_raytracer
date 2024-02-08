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
                risultato.colpito = True
                risultato.distanza = risultato_iter
                risultato.indice_sfera = indice
                risultato.punto_colpito = raggio.origine + raggio.dir_iterazione * risultato.distanza

        return risultato

class HitInfo:

    def __init__(self) -> None:
        self.colpito = False
        self.punto_colpito = np.array([.0,.0,.0])
        self.norma_colpito = np.array([.0,.0,.0])
        self.distanza = 0.0
        self.indice_sfera = 0
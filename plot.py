import pandas as pd
import numpy as np
import matplotlib.pyplot as plt

df = pd.read_csv('output.csv')
data = np.log10(df).T.values
plt.imshow(data[:int(data.shape[0]/2),:], aspect='auto', cmap='nipy_spectral')
plt.show()
print('ok')

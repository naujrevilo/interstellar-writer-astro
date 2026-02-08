"""
Generador simple de favicon basado en círculos concéntricos.

Requisitos:
- Pillow (PIL): pip install pillow

Salida:
- Crea un archivo 'favicon.png' en el directorio actual.
"""
from PIL import Image, ImageDraw

# Crear imagen 256x256 con transparencia
size = 256
img = Image.new('RGBA', (size, size), (0, 0, 0, 0))
draw = ImageDraw.Draw(img)

# Colores del logo (basados en el SVG)
azul_oscuro = (38, 34, 98, 255)  # #262262
azul = (28, 117, 188, 255)        # #1c75bc
rojo = (181, 39, 51, 255)         # #b52733
amarillo = (230, 142, 39, 255)    # #e68e27

center = size // 2

# Dibujar círculos concéntricos (del más grande al más pequeño)
# Círculo exterior azul oscuro
draw.ellipse([20, 20, size-20, size-20], fill=azul_oscuro)

# Círculo azul
draw.ellipse([50, 50, size-50, size-50], fill=azul)

# Círculo rojo
draw.ellipse([80, 80, size-80, size-80], fill=rojo)

# Círculo interior amarillo
draw.ellipse([110, 110, size-110, size-110], fill=amarillo)

# Guardar
img.save('favicon.png')
print("favicon.png creado exitosamente")

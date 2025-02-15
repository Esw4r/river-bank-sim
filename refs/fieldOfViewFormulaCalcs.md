

# Function: `entitiesInFOV()`

### Arguements:  
- `[all U8]`  

---

## Formulas:

### **Rotating to the Given Angle 'r'**  

1. **FOV Boundary Equation:**  
   $$
   y = \tan(FOVAngle) \cdot x
   $$
   - **Field of View Angle:**  
     $$
     FOVAngle = \frac{\pi}{3} \quad \text{(60°)}
     $$
   - After rotating by an angle $r$:  
     $$
     y = \tan(FOVAngle + r) \cdot x
     $$

2. **Horizontal Axis Equation:**  
   $$
   y = 0
   $$
   - After rotating by an angle $r$:  
     $$
     y = \tan(r) \cdot x
     $$

3. **Circular Boundary Equation:**  
   $$
   x^2 + y^2 = (radius)^2
   $$
   - **Radius:** $3$  
   - No need for rotation since it's already a circle.

---

## **Shifting to the Entity Position (a, b):**

1. **FOV Boundary Equation After Shifting:**  
   $$
   y - b = \tan(FOVAngle + r) \cdot (x - a)
   $$
   $$
   y = \tan(FOVAngle + r) \cdot (x - a) + b
   $$

2. **Horizontal Axis Equation After Shifting:**  
   $$
   y - b = \tan(r) \cdot (x - a)
   $$
   $$
   y = \tan(r) \cdot (x - a) + b
   $$

3. **Circular Boundary Equation After Shifting:**  
   $$
   (x - a)^2 + (y - b)^2 = (radius)^2
   $$
   $$
   y = \sqrt{(radius)^2 - (x - a)^2} + b
   $$

---

## **Final Parameters to Check for (px, py):**
1. $py \geq b$
2. $py \leq \tan\left(\frac{\pi}{3} - r\right) (px - a) + b$
3. $(px - a)^2 + (py - b)^2 \leq (radius)^2$  
   - **Given:** $\text{radius} = 3$

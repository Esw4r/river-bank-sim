

# Function: `entitiesInFOV()`

```{r, tidy=FALSE, eval=FALSE}

Algorithm isEntityInFOV:
	INput : Current Entity currEntity, Target Entity tarEntity
	REturns : Bool (if the Entity is in FOV or not)
	
	let x: u8 <- currEntity.get_posn()[0]
	let y: u8 <- currEntity.get_posn()[1]

	let x1: u8 <- tarEntity.get_posn()[0]
	let y1: u8 <- tarEntity.get_posn()[1]

	let radius: u8 <- currEntity.get_vision()
	let fovRange: u8 <- currEntity.get_fov()
	let fovAngle: u8 <- (currEntity.get_dir()*2)*(pi)/180      -- due to u8 the FOVAngle range from 0 to 179

	let entityInRadius: bool = (x1 - x)^2 + (y1 - y)^2 <= (radius)^2
	if (entityInRadius) then
		let entityInFOV: bool = (y1 <= tan(fovRange + fovAngle)(x1 - a) + y) && (y1 >= tan(fovAngle)(x1 - a) + y)
	
	return entityInFOV
	
```

### Arguements:  
- `[all U8]`  

---

## Formulas:

### **Rotating to the Given Angle 'r'**  

1. **FOV Boundary Equation:**  
   $$y = \tan(FOVAngle) \cdot x$$
   - **Field of View Angle:**  
     $$FOVAngle = \frac{\pi}{3} \quad \text{(60°)}$$
   - After rotating by an angle $r$:  
     $$y = \tan(FOVAngle + r) \cdot x$$

2. **Horizontal Axis Equation:**  
   $$y = 0$$
   - After rotating by an angle $r$:  
     $$y = \tan(r) \cdot x$$

3. **Circular Boundary Equation:**  
   $$x^2 + y^2 = (radius)^2$$
   - **Radius:** $3$
   - No need for rotation since it's already a circle.

---

## **Shifting to the Entity Position (a, b):**

1. **FOV Boundary Equation After Shifting:**  
   $$y - b = \tan(FOVAngle + r) \cdot (x - a)$$
   $$y = \tan(FOVAngle + r) \cdot (x - a) + b$$

2. **Horizontal Axis Equation After Shifting:**  
   $$y - b = \tan(r) \cdot (x - a)$$
   $$y = \tan(r) \cdot (x - a) + b$$

3. **Circular Boundary Equation After Shifting:**  
   $$(x - a)^2 + (y - b)^2 = (radius)^2$$
   $$y = \sqrt{(radius)^2 - (x - a)^2} + b$$

---

## **Final Parameters to Check for (px, py):**
1. $py \geq \tan(r) \cdot (px - a) + b$
2. $py \leq \tan\left(\frac{\pi}{3} + r\right) \cdot (px - a) + b$
3. $(px - a)^2 + (py - b)^2 \leq (\text{radius})^2$  
   - **Given:** $\text{Default radius} = 3$

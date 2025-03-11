package fm2gp

import (
	"math"
)

func EvaluatePolynomial(polynomial []float64, x float64) float64 {
	var acc float64 = 0
	for i := 0; i < len(polynomial); i++ {
		acc += polynomial[i] * math.Pow(x, float64(i))
	}
	return acc
}

func HornerEvaluatePolynomial(polynomial []float64, x float64) float64 {
	var acc float64 = 0
	for i := len(polynomial) - 1; i >= 0; i-- {
		acc = acc*x + polynomial[i]
	}
	return acc
}

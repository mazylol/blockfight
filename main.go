package main

import (
	"github.com/hajimehoshi/ebiten/v2"
	"github.com/hajimehoshi/ebiten/v2/vector"
	"image/color"
	"log"
)

const (
	screenWidth  = 640
	screenHeight = 900
)

type Game struct{}

func (g *Game) Update() error {
	return nil
}

type Player struct {
	posX int
	posY int
}

func (g *Game) Draw(screen *ebiten.Image) {
	vector.DrawFilledRect(screen, float32(0), float32(0), float32(50), float32(50), color.RGBA{
		R: 255,
		G: 0,
		B: 0,
		A: 1,
	}, true)
}

func (g *Game) Layout(outsideWidth, outsideHeight int) (int, int) {
	return screenWidth, screenHeight
}

func main() {
	ebiten.SetWindowSize(screenWidth, screenHeight)
	ebiten.SetWindowTitle("Hello, World!")
	if err := ebiten.RunGame(&Game{}); err != nil {
		log.Fatal(err)
	}
}

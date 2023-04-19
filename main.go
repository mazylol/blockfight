package main

import (
	"github.com/hajimehoshi/ebiten/v2"
	"github.com/hajimehoshi/ebiten/v2/vector"
	"image/color"
	"log"
)

const (
	screenWidth  = 1280
	screenHeight = 720
)

type Player struct {
	posX float32
	posY float32
}

type Game struct{}

var playerOne = Player{posX: 100, posY: 100}

func (g *Game) handleMovement() {
	if ebiten.IsKeyPressed(ebiten.KeyD) {
		playerOne.posX += 10
	} else if ebiten.IsKeyPressed(ebiten.KeyA) {
		playerOne.posX -= 10
	}
}

func (g *Game) Update() error {
	g.handleMovement()
	return nil
}

func (g *Game) Draw(screen *ebiten.Image) {
	vector.DrawFilledRect(screen, playerOne.posX, playerOne.posY, float32(50), float32(50), color.RGBA{
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

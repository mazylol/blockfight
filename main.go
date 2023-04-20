package main

import (
	"fmt"
	"github.com/hajimehoshi/ebiten/v2"
	"github.com/hajimehoshi/ebiten/v2/ebitenutil"
	"github.com/mazylol/blockfight/bfight"
	"image/color"
	"log"
)

const (
	screenWidth  = 1280
	screenHeight = 720
)

type Game struct{}

var playerOne = bfight.Player{PosX: 100, PosY: screenHeight - 55, Health: 100}
var playerTwo = bfight.Player{PosX: 200, PosY: screenHeight - 55, Health: 100}

func (g *Game) Update() error {
	playerOne.HandleMovement(ebiten.KeyA, ebiten.KeyD, ebiten.KeyW)
	playerTwo.HandleMovement(ebiten.KeyLeft, ebiten.KeyRight, ebiten.KeyUp)

	if playerOne.PosY < screenHeight-55 {
		playerOne.PosY += 5
	}

	if playerTwo.PosY < screenHeight-55 {
		playerTwo.PosY += 5
	}

	return nil
}

func (g *Game) Draw(screen *ebiten.Image) {
	playerOne.DrawPlayer(screen, color.RGBA{
		R: 255,
		G: 0,
		B: 0,
		A: 255,
	})
	playerTwo.DrawPlayer(screen, color.RGBA{
		R: 0,
		G: 255,
		B: 0,
		A: 255,
	})

	ebitenutil.DebugPrint(screen, fmt.Sprintf("Player 1: (%v, %v), Player 2: (%v, %v)\n", playerOne.PosX, playerOne.PosY, playerTwo.PosX, playerTwo.PosY))
}

func (g *Game) Layout(outsideWidth, outsideHeight int) (int, int) {
	return screenWidth, screenHeight
}

func main() {
	ebiten.SetWindowSize(screenWidth, screenHeight)
	ebiten.SetWindowTitle("Blockfight")
	if err := ebiten.RunGame(&Game{}); err != nil {
		log.Fatal(err)
	}
}

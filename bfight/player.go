package bfight

import (
	"github.com/hajimehoshi/ebiten/v2"
	"github.com/hajimehoshi/ebiten/v2/inpututil"
	"github.com/hajimehoshi/ebiten/v2/vector"
	"image/color"
)

type Player struct {
	PosX   float32
	PosY   float32
	Health int
	Jumped bool
}

func (p *Player) MoveRight() {
	p.PosX += 10
}

func (p *Player) MoveLeft() {
	p.PosX -= 10
}

func (p *Player) Jump() {
	p.PosY -= 150
}

func (p *Player) DrawPlayer(screen *ebiten.Image, color color.RGBA) {
	vector.DrawFilledRect(screen, p.PosX, p.PosY, float32(50), float32(50), color, false)
}

func (p *Player) HandleMovement(left ebiten.Key, right ebiten.Key, jump ebiten.Key) {
	if p.PosX < 1 {
		p.PosX = 10
	}

	if p.PosX > 1225 {
		p.PosX = 1225
	}

	if ebiten.IsKeyPressed(left) {
		p.MoveLeft()
	}
	if ebiten.IsKeyPressed(right) {
		p.MoveRight()
	}
	if inpututil.IsKeyJustPressed(jump) {
		p.Jump()
	}
}

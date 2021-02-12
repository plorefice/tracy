import { IScene } from "./scene";
import { SceneRenderer } from 'trtc';

export class Clock implements IScene {
	public readonly id = "chapter04";

	public readonly title = "Chapter 4: Matrix Transformations";

	public readonly width = 512;
	public readonly height = 512;

	public draw(sr: SceneRenderer, ctx: CanvasRenderingContext2D) {
		sr.draw(ctx, this.id, this.width, this.height);
	}
}

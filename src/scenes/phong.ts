import { IScene } from "./scene";
import { SceneRenderer } from 'trtc';

export class PhongSphere implements IScene {
	public readonly id = "chapter06";

	public readonly title = "Chapter 6: Light and Shading";

	public readonly width = 512;
	public readonly height = 512;

	public draw(sr: SceneRenderer, ctx: CanvasRenderingContext2D) {
		sr.draw(ctx, this.id, this.width, this.height);
	}
}

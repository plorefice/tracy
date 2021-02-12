import { IScene } from "./scene";
import { SceneRenderer } from 'trtc';

export class FlatSphere implements IScene {
	public readonly id = "chapter05";

	public readonly title = "Chapter 5: Ray-Sphere Intersections";

	public readonly width = 512;
	public readonly height = 512;

	public draw(sr: SceneRenderer, ctx: CanvasRenderingContext2D) {
		sr.draw(ctx, this.id, this.width, this.height);
	}
}

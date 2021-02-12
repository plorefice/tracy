import { IScene } from "./scene";
import { SceneRenderer } from 'trtc';

export class Projectile implements IScene {
    public readonly id = "chapter02";

    public readonly title = "Chapter 2: Drawing on a Canvas";

    public readonly width = 900;
    public readonly height = 550;

    public draw(sr: SceneRenderer, ctx: CanvasRenderingContext2D) {
        sr.draw(ctx, this.id, this.width, this.height);
    }
}

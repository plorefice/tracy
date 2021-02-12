import { SceneRenderer } from 'trtc';

export interface IScene {
  id: string;
  title: string;

  width: number;
  height: number;

  draw(sr: SceneRenderer, ctx: CanvasRenderingContext2D): void;
}
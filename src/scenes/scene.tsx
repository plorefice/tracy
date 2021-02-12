import React, { Component, RefObject } from 'react';

import { SceneRenderer } from 'trtc';

export interface IScene {
  readonly id: string;
  readonly title: string;
  readonly width: number;
  readonly height: number;

  draw(sr: SceneRenderer): void;
}

export class BaseScene extends Component implements IScene {
  public readonly id: string;
  public readonly title: string;
  public readonly width: number;
  public readonly height: number;

  canvas: RefObject<HTMLCanvasElement>;

  constructor(id: string, title: string, width: number, height: number) {
    super({})

    this.id = id;
    this.title = title;
    this.width = width;
    this.height = height;

    this.canvas = React.createRef();
  }

  public draw(renderer: SceneRenderer) {
    const ctx = this.canvas?.current?.getContext("2d")!
    renderer.draw(ctx, this.id, this.width, this.height);
  }

  public render() {
    /* Workaround for retina displays */
    const pixelRatio = window.devicePixelRatio || 1;
    const canvasStyle = {
      width: `${this.width / pixelRatio}px`,
      height: `${this.height / pixelRatio}px`,
    };

    return (
      <canvas
        ref={this.canvas}
        width={`${this.width}`}
        height={`${this.height}`}
        style={canvasStyle}
      />
    )
  }
}

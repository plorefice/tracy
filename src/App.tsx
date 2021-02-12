import React from 'react';

import { Projectile } from './scenes/projectile';
import { Clock } from './scenes/clock';
import { FlatSphere } from './scenes/flat-sphere';
import { PhongSphere } from './scenes/phong';

// In chapter order
const SCENES = [
  new Projectile(),
  new Clock(),
  new FlatSphere(),
  new PhongSphere(),
]

class App extends React.Component {
  render() {
    const sceneList = [];
    for (const scene of SCENES) {
      sceneList.push(
        <a href="#" className="list-group-item list-group-item-action">{scene.title}</a>
      );
    }

    return (
      <div className="container">
        <div className="row justify-content-center">
          <div className="list-group col-md-6" >
            {sceneList}
          </div >
        </div >
      </div >
    );
  }
}

export default App;

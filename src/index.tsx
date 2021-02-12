import * as React from 'react';
import { render } from 'react-dom';
import { BrowserRouter as Router, Link, Route } from 'react-router-dom';

import { SceneRenderer } from 'trtc';

import { IScene } from './scenes/scene';
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

const sceneMap = new Map(SCENES.map(s => [s.id, s] as [string, React.Component & IScene]));

interface ISceneProps {
  renderer: SceneRenderer,
  match: {
    params: {
      sceneId: string;
    }
  }
}

class Scene extends React.Component<ISceneProps> {
  scene: React.Component & IScene;

  constructor(props: ISceneProps) {
    super(props);
    this.scene = sceneMap.get(this.props.match.params.sceneId)!;
  }

  componentDidMount() {
    this.scene.draw(this.props.renderer);
  }

  render() {
    return (
      <div className="container text-center">
        {this.scene.render()}
      </div>
    );
  }
}

class SceneSelector extends React.Component {
  render() {
    return (
      <div className="container">
        <div className="row justify-content-center">
          <ul className="list-group col-md-6">
            {
              SCENES.map(scene => (
                <li className="list-group-item list-group-item-action" key={scene.id}>
                  <Link to={`/scene/${scene.id}`}>
                    {scene.title}
                  </Link>
                </li >
              ))
            }
          </ul >
        </div >
      </div >
    );
  }
}

class Title extends React.Component {
  render() {
    return (
      <div className="py-5 text-center container">
        <div className="row">
          <h1 className="fw-light">The Ray Tracer Challenge</h1>
          <h3><small className="lead text-muted">An experiment in Rust and WebAssembly</small></h3>
        </div>
      </div>
    )
  }
}

interface IAppProps {
  renderer: SceneRenderer,
}

class App extends React.Component<{}, IAppProps> {
  componentDidMount() {
    import('trtc').then(trtc => {
      trtc.init();
      this.setState({ renderer: new trtc.SceneRenderer });
    });
  }

  render() {
    return (
      <Router>
        <Title />
        <Route exact path="/" component={SceneSelector} />
        {
          this.state && (
            <Route path="/scene/:sceneId" render={(props) => (
              <Scene {...props} renderer={this.state.renderer} />
            )} />
          )
        }
      </Router>
    )
  }
}

render(<App />, document.getElementById('root'))

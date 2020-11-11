import '../stylesheet.css';
import { Animated } from 'react-web-animation';
import React from 'react'; 

class Overlay extends React.Component {
    constructor(props) {
        super(props)
        
        this.state = {"finished": false}
    }

    getKeyFrames() {
        return [
            {opacity: 0},
            {opacity: 0.6},
        ];
    }

    getTiming() {
        return {
            duration: 500,
            direction: this.props.hidden ? "normal" : "reverse",
            fill: "forwards"
        };
    }

    componentDidUpdate(prevProps) {
        if (this.props.hidden != prevProps.hidden) {
            this.setState({"finished": false})
        }
    }

    onFinish() {
        // Do not hide div when blurred
        if (!this.props.hidden) {
            this.setState({"finished": true})
        }
    }

    render() {
        return (
            <Animated.div hidden={this.state.finished} class="overlay" keyframes={this.getKeyFrames()} timing={this.getTiming()} onFinish={this.onFinish.bind(this)}/>
        )
    }
}

export default Overlay;
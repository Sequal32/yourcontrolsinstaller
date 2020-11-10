import React from 'react';
import '../stylesheet.css';

class FeatureEntry extends React.Component {
    constructor(props) {
        super(props)
        this.state = {"checked": false}
    }

    onHover() {
        this.setState({"hover": true})
    }

    onLeave() {
        this.setState({"hover": false})
    }

    onClick() {
        const newValue = !this.state.checked
        this.setState({"checked": newValue})

        if (this.props.callback) {
            this.props.callback(newValue)
        }
    }

    generateClassName() {
        var classString = "custom-checkbox"
        
        if (this.state.hover) {
            classString += " checkbox-hover"
        }

        return classString
    }

    render() {
        return (
            <div class="feature-div">
                <div class={this.generateClassName()} onMouseEnter={this.onHover.bind(this)} onMouseLeave={this.onLeave.bind(this)} onMouseDown={this.onClick.bind(this)} checked={this.state.checked}>
                    <img src="check.png" class="checkbox-image" hidden={!this.state.checked}/>
                </div>
                <span class="feature-text">{this.props.name}</span>
            </div>
        )
    };
}

class OptionalFeatures extends React.Component {
    constructor(props) {
        super(props)
        
        this.features = []
        this.selectedFeatures = new Set()
    }

    componentDidUpdate() {
        this.selectedFeatures.clear()
    }

    selectFeature(featureName, selected) {
        if (selected) {
            this.selectedFeatures.add(featureName)
        } else {
            this.selectedFeatures.delete(featureName)
        }

        if (this.props.callback) {
            this.props.callback(this.selectedFeatures)
        }
    }

    render() {
        this.features = this.props.featureList != null ? this.props.featureList.map((featureName) => <FeatureEntry name={featureName} callback={this.selectFeature.bind(this, featureName)}/>) : []

        return (
            <div class="feature-list">
                <h3 class="feature-list-text">
                    Optional Features
                </h3>
                <div class="feature-list-grid">
                    {this.features}
                </div>
            </div>
        );
    }
}

export default OptionalFeatures;
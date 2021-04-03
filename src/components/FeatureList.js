import React from 'react';
import '../stylesheet.css';

class FeatureEntry extends React.Component {
    constructor(props) {
        super(props)
        this.state = {"hover": false}
    }

    onHover() {
        this.setState({"hover": true})
    }

    onLeave() {
        this.setState({"hover": false})
    }

    onClick() {
        if (this.props.callback) {
            this.props.callback()
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
                    <img src="check.png" class="checkbox-image" alt="" hidden={!this.props.checked}/>
                </div>
                <span class="feature-text">{this.props.name}</span>
            </div>
        )
    };
}

class OptionalFeatures extends React.Component {
    constructor(props) {
        super(props)
        
        this.groupMap = {}
        this.selectedFeatures = new Set()
    }

    componentDidUpdate(prevProps) {
        if (prevProps.features !== this.props.features) {
            this.selectedFeatures.clear()
            this.groupMap.clear()
        }
    }

    selectFeature(featureName, featureGroup) {
        if (this.selectedFeatures.has(featureName)) {

            this.selectedFeatures.delete(featureName)

        } else {

            // Make sure only one from each group gets selected
            if (featureGroup) {
                const groupSelectedFeature = this.groupMap[featureGroup]
                // Remove the other selected feature from the group
                if (groupSelectedFeature) {
                    this.selectedFeatures.delete(groupSelectedFeature)
                }
                // Insert the new selected feature into the group
                this.groupMap[featureGroup] = featureName
            }
            
            this.selectedFeatures.add(featureName)
        }

        if (this.props.callback) {
            this.props.callback(this.selectedFeatures)
        }

        this.forceUpdate()
    }

    render() {
        this.features = this.props.featureList != null ? this.props.featureList.map((feature) => <FeatureEntry name={feature.name} callback={this.selectFeature.bind(this, feature.name, feature.group)} checked={this.selectedFeatures.has(feature.name)}/>) : []

        return (
            <div class="feature-list-grid">
                {this.features}
            </div>
        );
    }
}

export default OptionalFeatures;
export {FeatureEntry, OptionalFeatures};
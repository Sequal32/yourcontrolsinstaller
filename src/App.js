import './stylesheet.css';
import React from 'react';
import { promisified, invoke } from 'tauri/api/tauri'

import Dialog from './components/Dialog'
import DirectoryEntry from './components/DirectoryEntry'
import OptionalFeatures from './components/FeatureList'
import Overlay from './components/Overlay'

class App extends React.Component {
    constructor(props) {
        super(props)

        this.state = {
            programDirectory: "Unknown",
            packageDirectory: "Unknown",
            browsing: false,
            installing: false,
            
            currentDialog: {
                title: "",
                description: "",
                buttonText: "",
            },
            dialogActive: false
        }
        this.selectedFeatures = []
    }

    componentDidMount() {
        promisified({
            cmd: "startup"
        }).then((args) => {
            if (args.releaseData !== null) {
                const date = new Date(args.releaseData.date * 1000)

                this.setState({
                    currentDialog: {
                        title: "Release " + args.releaseData.tagName,
                        description: `Released on ${date.toDateString()}`,
                        buttonText: "OK",
                    },
                    dialogActive: true
                })
            }
            

            this.setState(args)
        })
    }

    featuresCallback(selectedFeatures) {
        this.selectedFeatures = Array.from(selectedFeatures)
    }

    promptInstall() {
        this.setState({"installing": true})

        promisified({
            cmd: "install",
            features: this.selectedFeatures
        }).then(() => {

            this.setState({
                currentDialog: {
                    title: "Installation Successful",
                    description: "The program has been successfully installed. Get flying!",
                    buttonText: "OK"
                },
                dialogActive: true
            })

        }).catch((errorMessage) => {

            this.setState({
                currentDialog: {
                    title: "Installation Failed",
                    description: errorMessage + "\n\nMore info available in Log.txt.",
                    buttonText: "OK",
                },
                dialogActive: true
            })

        }).finally(() => {
            this.setState({installing: false})
        })
    }

    promptUninstall() {
        this.setState({"uninstalling": true})

        promisified({
            cmd: "uninstall",
        }).then(() => {

            this.setState({
                currentDialog: {
                    title: "Uninstalling Successful",
                    description: "The community package has been successfully uninstalled. If you'd like to remove the application, delete the program folder.",
                    buttonText: "OK"
                },
                dialogActive: true
            })

        }).catch((errorMessage) => {

            this.setState({
                currentDialog: {
                    title: "Uninstalling Failed",
                    description: errorMessage + "\n\nMore info available in Log.txt.",
                    buttonText: "OK",
                },
                dialogActive: true
            })

        }).finally(() => {
            this.setState({uninstalling: false})
        })
    }

    onDirectoryBrowse(type) {
        if (this.state.browsing) {return}

        this.setState({browsing: true})

        promisified({
            cmd: "browse",
            browse_for: type
        }).then((location) => {
            this.setState({[type + "Directory"]: location})
        }).finally(() => {
            this.setState({browsing: false})
        })
    }

    dialogButtonClicked() {
        this.setState({dialogActive: false})
    }

    render() {
        return (
            <div>
                    <img class="logo-image" src="logo.png"/>
                    <DirectoryEntry title="Program Installation Directory" location={this.state.programDirectory} onBrowse={this.onDirectoryBrowse.bind(this, "program")}/>
                    <DirectoryEntry title="Community Packages Directory" location={this.state.packageDirectory} onBrowse={this.onDirectoryBrowse.bind(this, "package")}/>
                    <OptionalFeatures featureList={this.state.featureList} callback={this.featuresCallback.bind(this)}/>
                  
                    <button class="generic-button install-button" onClick={this.promptInstall.bind(this)} disabled={this.state.installing}>{this.state.installing ? "Installing" : "Install"}</button>
                    <button class="generic-button uninstall-button" onClick={this.promptUninstall.bind(this)} disabled={this.state.uninstalling}>{this.state.uninstalling ? "Uninstalling" : "Uninstall"}</button>
                  
                    <Overlay hidden={this.state.dialogActive}/>
                    <Dialog hidden={this.state.dialogActive} title={this.state.currentDialog.title} description={this.state.currentDialog.description} buttonText={this.state.currentDialog.buttonText} callback={this.dialogButtonClicked.bind(this)}/>
            </div>
        );
    }
}

export default App;
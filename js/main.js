#!/usr/bin/env node
const { resolve, dirname } = require('path');
const exec = require('child_process').exec
const prompts = require('prompts');

// JS wrapper of doctave --> https://github.com/Doctave/doctave

/**
 * @param {...string} cmd
 */
const execCommand = (cmd) => {
    const child = exec(cmd);
    child.stdout.on('data', (data) => {
        console.log(new Date(), data.toString());
    });

    child.stderr.on('data', (data) => {
        console.error(`stderr: ${data}`);
    });
}

(async () => {
    console.info(`Doctave site generator 📚`);

    if (process.platform !== 'darwin') {
        console.info(`The js runner has no support of your platform: [${process.platform}]`);
        process.exit(0);
    }


    const doctaveExec = resolve(dirname(require.main.filename),'./bin/doctave');
    const commands = {
        init: {cmd: `${doctaveExec} init`, desc: 'Create an initial docs site'},
        serve: {cmd: `${doctaveExec} serve -o`, desc: 'Preview your site locally (http://0.0.0.0:4001/)'},
        build: {cmd: `${doctaveExec} build`, desc: 'Build the site static assets'},
    }

    const response = await prompts([
        {
            type: 'autocomplete',
            name: 'commands',
            message: 'What would you like to do?',
            choices: Object.entries(commands).map((entry) => ({
                title: entry[0],
                value: entry[0],
                description: entry[1].desc,
            })),
            initial: 0,
        },
    ]);

    switch (response.commands) {
        case 'init':
            execCommand(commands.init.cmd);
            break;
        case 'serve':
            execCommand(commands.serve.cmd);
        break
        case 'build':
            execCommand(commands.build.cmd);
            break
        default:
            console.log('no selection');
    }

})();







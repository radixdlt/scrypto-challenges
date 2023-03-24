import { publish } from 'gh-pages';

publish(
    'build', // path to public directory
    {
        branch: 'gh-pages',
        repo: 'git@github.com:backyard-coder/flashyfi.git',
        user: {
            name: 'backyard-coder',
            email: '94840973+backyard-coder@users.noreply.github.com'
        },
        dotfiles: true
    },
    () => {
        console.log('Deploy Complete!');
    }
);
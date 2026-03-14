import pluginVue from 'eslint-plugin-vue';
import vueParser from 'vue-eslint-parser';
import tseslintParser from '@typescript-eslint/parser';

export default [
  {
    ignores: ['dist', 'node_modules', 'coverage']
  },
  {
    files: ['**/*.vue'],
    languageOptions: {
      parser: vueParser,
      parserOptions: {
        parser: tseslintParser,
        ecmaVersion: 'latest',
        sourceType: 'module'
      }
    },
    plugins: {
      vue: pluginVue
    },
    rules: {
      ...pluginVue.configs['flat/essential'].rules,
      'vue/multi-word-component-names': 'off'
    }
  }
];

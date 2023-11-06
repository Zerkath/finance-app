import { test, expect, beforeAll } from 'vitest';
import { randomFillSync } from 'crypto';
import { tick } from 'svelte';
import { render, fireEvent } from '@testing-library/svelte';

import { mockIPC } from '@tauri-apps/api/mocks';
import CategoryComponent from './CategoryComponent.svelte';

// jsdom doesn't come with a WebCrypto implementation
beforeAll(() => {
  Object.defineProperty(window, 'crypto', {
    value: {
      // @ts-ignore
      getRandomValues: (buffer) => {
        return randomFillSync(buffer);
      }
    }
  });
});

const getScreen = () => {
  const screen = render(CategoryComponent, { label: 'Test', categoryId: 10 });
  const editOrCancelButton = screen.getByTestId('category-modify-action');
  const saveOrDeleteButton = screen.getByTestId('category-apply-action');
  const textField = screen.getByTestId('category-input');

  return { editOrCancelButton, saveOrDeleteButton, textField };
};

test('Initially should have two buttons available', async () => {
  const { editOrCancelButton, saveOrDeleteButton } = getScreen();
  expect(editOrCancelButton.textContent).toBe('Edit');
  expect(saveOrDeleteButton.textContent).toBe('Delete');
});

test('Should disable textfield', async () => {
  const { textField } = getScreen();

  expect(textField.disabled).toBe(true);
  expect(textField.value).toBe('Test');
});

test('After clicking edit, should have cancel and save available', async () => {
  const { editOrCancelButton, saveOrDeleteButton } = getScreen();

  await fireEvent.click(editOrCancelButton);

  expect(editOrCancelButton.textContent).toBe('Cancel');
  expect(saveOrDeleteButton.textContent).toBe('Save');
});

test('Editing then cancelling should revert back to original value', async () => {
  const { editOrCancelButton, saveOrDeleteButton, textField } = getScreen();
  expect(textField.disabled).toBe(true);
  expect(textField.value).toBe('Test');
  await fireEvent.click(editOrCancelButton);

  await fireEvent.input(textField, { target: { value: 'Changed' } });

  expect(textField.disabled).toBe(false);
  expect(
    textField.value,
    'Inputting new value should affect the value stored by this field'
  ).toBe('Changed');

  await fireEvent.click(editOrCancelButton);
  expect(editOrCancelButton.textContent).toBe('Edit');
  expect(saveOrDeleteButton.textContent).toBe('Delete');
  expect(textField.disabled).toBe(true);
  expect(textField.value, 'The cancel did not update the fields value').toBe(
    'Test'
  );
});

const expectedId = 10;

let saveCount = 0;
let deleteCount = 0;

mockIPC((cmd, args) => {
  // simulated rust command called "add" that just adds two numbers
  if (cmd === 'update_category_label') {
    test('The save should pass values to backend', () => {
      expect(args.id).toBe(expectedId);
      expect(args.label).toBe('Changed');
    });
    saveCount++;
    return undefined;
  } else if (cmd === 'delete_category') {
    test('The delete should pass values to backend', () => {
      expect(args.id).toBe(expectedId);
    });
    deleteCount++;
    return undefined;
  }
});

test('The save did not update the fields value', async () => {
  const { editOrCancelButton, saveOrDeleteButton, textField } = getScreen();
  await fireEvent.click(editOrCancelButton);

  await fireEvent.change(textField, { target: { value: 'Changed' } });

  await fireEvent.click(saveOrDeleteButton);
  expect(textField.value, 'The save did not update the fields value').toBe(
    'Changed'
  );
  await fireEvent.click(saveOrDeleteButton);
  await tick();
  expect(saveCount).toBe(1);
  expect(deleteCount).toBe(1);
});

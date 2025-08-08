import React, { useEffect, useState } from 'react';
import { Text, View, StyleSheet, FlatList, TextInput, Button } from 'react-native';

export default function App() {
  const [entries, setEntries] = useState([]);
  const [newNotes, setNewNotes] = useState('');

  const fetchEntries = () => {
    fetch('http://localhost:3000/wear-entries')
      .then(res => res.json())
      .then(setEntries)
      .catch(console.error);
  };

  useEffect(() => {
    fetchEntries();
  }, []);

  const addEntry = () => {
    const payload = {
      clothing: [
        { id: 1, name: 'Camisa', category: 'Top' },
      ],
      weather: {
        timestamp: new Date().toISOString(),
        temperature_c: 20,
        humidity_pct: 50,
      },
      comfort: 'Comfortable',
      notes: newNotes,
    };

    fetch('http://localhost:3000/wear-entries', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(payload),
    })
      .then(res => res.json())
      .then(entry => {
        setEntries([...entries, entry]);
        setNewNotes('');
      })
      .catch(console.error);
  };

  const updateEntry = (id, entry) => {
    fetch(`http://localhost:3000/wear-entries/${id}`, {
      method: 'PUT',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(entry),
    })
      .then(res => res.json())
      .then(updated => {
        setEntries(entries.map(e => (e.id === id ? updated : e)));
      })
      .catch(console.error);
  };

  const EntryItem = ({ item }) => {
    const [notes, setNotes] = useState(item.notes || '');
    return (
      <View style={styles.item}>
        <Text style={styles.text}>{item.clothing.map(c => c.name).join(', ')}</Text>
        <Text style={styles.subtext}>
          {`${item.weather.temperature_c}°C, ${item.weather.humidity_pct}% - ${item.comfort}`}
        </Text>
        <TextInput
          value={notes}
          onChangeText={setNotes}
          placeholder="Notas"
          style={styles.input}
        />
        <Button title="Actualizar" onPress={() => updateEntry(item.id, { ...item, notes })} />
      </View>
    );
  };

  return (
    <View style={styles.container}>
      <View style={styles.newEntry}>
        <TextInput
          value={newNotes}
          onChangeText={setNewNotes}
          placeholder="Notas"
          style={styles.input}
        />
        <Button title="Agregar" onPress={addEntry} />
      </View>
      <FlatList
        data={entries}
        keyExtractor={item => item.id.toString()}
        renderItem={({ item }) => <EntryItem item={item} />}
        ListEmptyComponent={<Text style={styles.text}>No entries</Text>}
      />
    </View>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
    backgroundColor: '#121212',
    paddingTop: 50,
    paddingHorizontal: 16,
  },
  text: {
    color: '#fff',
    fontSize: 16,
  },
  subtext: {
    color: '#bbb',
    fontSize: 14,
  },
  item: {
    marginBottom: 16,
  },
  input: {
    backgroundColor: '#fff',
    padding: 4,
    marginVertical: 4,
  },
  newEntry: {
    marginBottom: 24,
  },
});
